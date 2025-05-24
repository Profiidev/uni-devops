<script lang="ts">
  import { password_login, password_register } from '$lib/password.svelte';

  let email = $state('');
  let name = $state('');
  let password = $state('');

  const register = async () => {
    console.log(await password_register(email, password, name));
  };

  const login = async () => {
    let ret = await password_login(email, password);
    if (ret === undefined) {
      console.log('Login successful');
      window.location.href = '/';
    } else {
      console.log('There was an Error while signing in:', ret);
    }
  };
</script>

<h1>login</h1>
<input type="text" placeholder="email" bind:value={email} />
<input type="text" placeholder="name" bind:value={name} />
<input type="text" placeholder="password" bind:value={password} />
<br />
<button onclick={register}>register</button>
<br />
<button onclick={login}>login</button>
