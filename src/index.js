import init, { extract_comments } from 'wasm-lib'

document.addEventListener('DOMContentLoaded', async () => {
  await init()
  const input = document.querySelector('textarea')
  const outputList = document.querySelector('.output__list')

  input.addEventListener('input', (e) => {
    /* if (e.key === 'Enter' && !e.shiftKey) { */
    /* e.preventDefault() */
    console.time('rust')
    const extracted = extract_comments(input.value)
    const rustTimer = console.timeEnd('rust')
    console.log('Rust Timer', rustTimer)

    console.time('js')
    const extractedJs = extractComment(input.value)
    const jsTimer = console.timeEnd('js')
    console.log('Js Timer', jsTimer)

    /* outputList.innerHTML = '' */
    /* extracted.forEach((comment) => { */
    /*   outputList.innerHTML += `<li class="output__item">${comment}</li>` */
    /* }) */
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
