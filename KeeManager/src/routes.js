import SelectDatabase from './routes/SelectDatabase.svelte';
import CreateDatabase from './routes/CreateDatabase.svelte';
import Main from './routes/Main.svelte';
import './global.css';

export default {
  '/': SelectDatabase,
  '/create': CreateDatabase,
  '/main': Main,
};
