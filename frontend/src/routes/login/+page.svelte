<script lang="ts">
  import { password_login, password_register } from '$lib/password.svelte';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { page } from '$app/state';

  let loginToggle = $state(true);
  let email = $state('');
  let name = $state('');
  let password = $state('');
  let error = $state('');
  let redirect = '/';

  onMount(() => {
    redirect = page.url.searchParams.get('redirectTo') || '/';
  });

  const register = async () => {
    let res = await password_register(email, password, name);
    if (res === undefined) {
      console.log('Registration successful');
      goto(redirect);
    } else {
      console.log('There was an Error while signing up:', res);
      error =
        'There was an error during registration. Please check your input.';
    }
  };

  const login = async () => {
    let res = await password_login(email, password);
    if (res === undefined) {
      console.log('Login successful');
      goto(redirect);
    } else {
      console.log('There was an Error while signing in:', res);
      error = 'There was an error during login. Please check your input.';
    }
  };
</script>

<main>
  <h1>Dealing With DevOps</h1>

  <div class="login-form">
    <h2>{loginToggle ? 'Sign in to DevOps' : 'Account Creation'}</h2>

    <div class="input-container">
      <p>Email:</p>
      <input type="text" placeholder="" bind:value={email} />
    </div>

    {#if !loginToggle}
      <div class="input-container">
        <p>Name:</p>

        <input type="text" placeholder="" bind:value={name} />
      </div>
    {/if}

    <div class="input-container">
      <p>Password:</p>
      <input type="password" placeholder="" bind:value={password} />
    </div>

    {#if loginToggle}
      <button class="primary-button" onclick={login}>Sign In</button>
    {:else}
      <button class="primary-button" onclick={register}>Sign Up</button>
    {/if}

    <div class="toggle-container">
      <p>
        {loginToggle ? 'New to Devops?' : 'Already have an account?'}
      </p>
      <button
        class="toggle-button"
        onclick={() => ((loginToggle = !loginToggle), (error = ''))}
      >
        {loginToggle ? 'Sign up' : 'Sign in'}
      </button>
    </div>

    {#if error}
      <p class="error-message">{error}</p>
    {/if}
  </div>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 2rem;
    gap: 2rem;
  }

  h1 {
    font-size: 2rem;
  }

  h2 {
    font-size: 1.5rem;
  }

  p {
    font-size: 1rem;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    border-radius: 15px;
    padding: 3.5rem;
    gap: 1rem;
    background-color: var(--surface-background);
    width: min(90vw, 500px);
  }

  input {
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 0.2rem;
    width: 100%;
  }

  .primary-button {
    padding: 0.5rem 1rem;
    background-color: var(--primary-color);
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .toggle-button {
    background-color: transparent;
    border: none;
    color: var(--primary-color);
    cursor: pointer;
  }

  .error-message {
    color: red;
  }
</style>
