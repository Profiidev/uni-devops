<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';
  import { test_token } from '$lib/password.svelte';
  import { browser } from '$app/environment';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';

  let { children } = $props();

  onMount(() => {
    test_token().then((valid) => {
      if (!valid && browser) {
        let url = page.url.pathname;
        if (url !== '/login') {
          goto('/login');
        }
      }
    });
  });
</script>

{@render children()}
