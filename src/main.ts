import App from "./App.svelte";
import  './tailwind.css';

const app = new App({
  target: document.getElementById("app"),
});

// now we can call our Command!
// Right-click the application background and open the developer tools.
// You will see "Hello, World!" printed in the console!

export default app;
