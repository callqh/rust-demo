import.meta.hot.accept(mod=>{
  const a = 1
});
import.meta.hot?.dispose()
if (import.meta.hot) {
  import.meta.hot.accept(mod => {
    const div = document.getElementById(id);
    div?.appendChild(mod.createChild());
  });
  import.meta.hot.dispose(() => {
    const div = document.getElementById(id);
    if (div) {
      while (div.firstChild) {
        console.log('dispose', div.firstChild);
        div.removeChild(div.firstChild);
      }
    }
  });
}
