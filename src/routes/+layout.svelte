<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { initEventListeners } from '$lib/events/listener';

  onMount(() => {
    initEventListeners();
  });
</script>

<slot />
