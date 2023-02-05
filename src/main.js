
// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri

invoke('greet', {name: 'World'})
  .then(res => {
    console.log(res)
  })