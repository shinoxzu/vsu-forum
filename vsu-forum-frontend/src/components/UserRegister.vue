<template>
  <div class="register">
    <h2>Регистрация</h2>
    <form @submit.prevent="register">
      <label for="username">Логин:</label>
      <input id="username" v-model="username" required />

      <label for="password">Пароль:</label>
      <input id="password" type="password" v-model="password" required />

      <button type="submit">Зарегистрироваться</button>
    </form>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  name: 'UserRegister',
  setup() {
    const username = ref('');
    const password = ref('');

    async function register() {
      try {
        const response = await fetch('http://localhost:3000/users/register', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ login: username.value, password: password.value })
        });
        if (response.ok) {
          const data = await response.json();
          localStorage.setItem('token', data.token);
          console.log("Регистрация успешна!", data);
        } else {
          console.error("Ошибка при регистрации");
        }
      } catch (error) {
        console.error("Ошибка сети:", error);
      }
    }

    return { username, password, register };
  }
};
</script>

<style scoped>
/* Добавьте стили, если нужно */
</style>