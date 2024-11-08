<template>
  <div class="login">
    <h2>Вход</h2>
    <form @submit.prevent="login">
      <label for="username">Логин:</label>
      <input id="username" v-model="username" required />

      <label for="password">Пароль:</label>
      <input id="password" type="password" v-model="password" required />

      <button type="submit">Войти</button>
    </form>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  name: 'UserLogin',
  setup() {
    const username = ref('');
    const password = ref('');

    async function login() {
      try {
        const response = await fetch('http://localhost:3000/users/login', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ login: username.value, password: password.value })
        });
        if (response.ok) {
          const data = await response.json();
          localStorage.setItem('token', data.token);
          console.log("Вход успешен!", data);
        } else {
          console.error("Ошибка при входе");
        }
      } catch (error) {
        console.error("Ошибка сети:", error);
      }
    }

    return { username, password, login };
  }
};
</script>

<style scoped>
/* Добавьте стили, если нужно */
</style>