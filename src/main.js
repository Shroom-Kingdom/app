import App from './App.svelte';
import { checkWebGL2 } from './modules/webgl2';
import NoWebGL2App from './modules/webgl2/NoWebGL2App.svelte';

const hasWebGL2Support = checkWebGL2();

const app = hasWebGL2Support
  ? new App({
      target: document.querySelector('#root'),
      props: {
        name: 'world'
      }
    })
  : new NoWebGL2App({
      target: document.querySelector('#root')
    });

export default app;
