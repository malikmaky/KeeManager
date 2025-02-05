<script>
  import Router from 'svelte-spa-router';
  import routes from '../routes';
  import '../global.css';

  document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<main>
  <Router {routes} />
</main>