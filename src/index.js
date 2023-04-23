import init, { extract_comments } from 'wasm-lib'

document.addEventListener('DOMContentLoaded', async () => {
  await init()
  const input = document.querySelector('textarea')
  const outputList = document.querySelector('.output__list')

  input.addEventListener('input', (e) => {
    /* if (e.key === 'Enter' && !e.shiftKey) { */
    /* e.preventDefault() */
    const extracted = extract_comments(input.value)

    outputList.innerHTML = ''
    extracted.forEach((comment) => {
      outputList.innerHTML += `<li class="output__item">${comment}</li>`
    })
    /* } */
  })
})
