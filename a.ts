import { a } from './dep';

console.log(a);
const id ='cls';



if (import.meta.hot) {
  // self accept without reload the page
  import.meta.hot.accept(mod => {
    const div = document.getElementById(id);
    div?.appendChild(mod.createChild());
  });
  import.meta.hot.dispose(() => {
    // remove all children of the div
    const div = document.getElementById(id);
    
    if (div) {
      while (div.firstChild) {
        console.log('dispose', div.firstChild);
        div.removeChild(div.firstChild);
      }
    }
  });
}
