// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri


// invoke the `read` command
// invoke({
//   cmd: 'read',
//   path: 'path/to/file'
// }).then((res) => {
//   console.log(res)
// }


let simplemde = new SimpleMDE({ 
  element: document.getElementById("editor"),
  placeholder: "Type here...",
});

