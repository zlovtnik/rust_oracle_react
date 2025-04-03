import './styles/theme.css'
// Use Carbon's dark theme as the base for our custom dark theme
import 'carbon-components-svelte/css/g100.css';
import App from './App.svelte'
import { mount } from 'svelte'

// Svelte 5 uses mount function instead of constructor pattern
mount(App, {
  target: document.getElementById('app') as HTMLElement,
})

export default App
