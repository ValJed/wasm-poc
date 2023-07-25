import init, { extract_comments, instantiate_rust_listener } from 'wasm-lib'

document.addEventListener('DOMContentLoaded', async () => {
  await init()
  instantiate_rust_listener()
  const input = document.querySelector('.block__js .input')
  const outputList = document.querySelector('.block__js .output__list')

  input.addEventListener('input', (e) => {
    console.time('jsExtractedCode')
    const extracted = extractComment(input.value)
    console.timeEnd('jsExtractedCode')

    console.time('jsManipulateDom')
    outputList.innerHTML = ''
    extracted.forEach((comment) => {
      outputList.innerHTML += `<li class="output__item">${comment}</li>`
    })
    console.timeEnd('jsManipulateDom')
  })
})

function extractComment(code) {
  let isInSingleStr = false
  let isInDoubleStr = false
  let prevChar = ''
  let isComment = false
  let isLineComment = false
  const comments = []
  let currentComment = ''

  for (const char of Array.from([...code])) {
    if (!isComment && !isLineComment) {
      if (char === "'" && !isInDoubleStr) {
        isInSingleStr = !isInSingleStr
      } else if (char === '"' && !isInSingleStr) {
        isInDoubleStr = !isInDoubleStr
      }
    }

    if (isInSingleStr || isInDoubleStr) {
      continue
    }

    if (prevChar === '/') {
      if (char === '/') {
        if (!isLineComment) {
          isLineComment = true
          continue
        }
      } else if (char === '*') {
        if (!isComment) {
          isComment = true
          continue
        }
      }
    }

    if (isLineComment && char === '\n') {
      comments.push(currentComment)
      currentComment = ''
      isLineComment = false
      continue
    }

    if (prevChar === '*' && char == '/' && isComment) {
      currentComment = currentComment.substring(0, currentComment.length - 1)
      comments.push(currentComment)
      currentComment = ''
      isComment = false
      continue
    }

    if (isComment || isLineComment) {
      currentComment = currentComment + char
    }

    prevChar = char
  }

  return comments.filter((comment) => comment)
}
