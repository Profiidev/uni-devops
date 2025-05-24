<script lang="ts">
  import { password_login, password_register } from '$lib/password.svelte';
  import { goto } from '$app/navigation';

  let loginToggle = $state(true);
  let email = $state('');
  let name = $state('');
  let password = $state('');
  let error = $state('');

  const register = async () => {
    let res = await password_register(email, password, name);
    if (res === undefined) {
      console.log('Registration successful');
      goto('/');
    } else {
      console.log('There was an Error while signing up:', res);
      error = "Es gab einen Fehler bei der Registrierung. Bitte überprüfe deine Eingaben.";
    }
  };

  const login = async () => {
    let res = await password_login(email, password);
    if (res === undefined) {
      console.log('Login successful');
      goto('/');
    } else {
      console.log('There was an Error while signing in:', res);
      error = "Es gab einen Fehler beim Anmelden. Bitte überprüfe deine Eingaben.";
    }
  };
</script>

<main>
  <h1>Dealing With DevOps</h1>

  <div class="login-form">
    <h2>{loginToggle ? 'Anmelden' : 'Account erstellen'}</h2>

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
      <button class="primary-button" onclick={login}>Anmelden</button>
    {:else}
      <button class="primary-button" onclick={register}>Account erstellen</button>
    {/if}

    <div class="toggle-container">
      <p>
        {loginToggle
          ? 'Noch keinen Account?'
          : 'Bereits einen Account?'}
      </p>
      <button class="toggle-button" onclick={() => (loginToggle = !loginToggle, error = '')}>
        {loginToggle ? 'Registrieren' : 'Anmelden'}
      </button>
  </div>

    {#if error}
      <p class="error-message">{error}</p>
    {/if}
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
