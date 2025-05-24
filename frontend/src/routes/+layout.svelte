<script lang="ts">
  import { PUBLIC_BACKEND_URL } from '$env/static/public';
  import { onMount } from 'svelte';
  import '../app.css';

  let { children } = $props();
</script>

{@render children()}
