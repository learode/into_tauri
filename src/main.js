
// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri

let $input;
let $button;
let $output;



  window.addEventListener("DOMContentLoaded", () => {
    $input = document.querySelector('#name');
    $button = document.querySelector('#greet');
    $output = document.querySelector('#greeting-msg');

    $button.addEventListener("click", () => {
      invoke("greet", { name: $input.value }).then((res) => {
        $output.innerHTML = res;
      });
    });
  })