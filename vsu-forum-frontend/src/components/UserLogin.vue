
  <script setup>
  
import Button from 'primevue/button';


import InputText from 'primevue/inputtext';
    </script>

<template>
  <div class="login">
    
    <form @submit.prevent="login" class="form-login">
     <h2>Вход в аккаунт</h2>
    <InputText v-model="username" id="username" placeholder="Username" required style="margin-bottom: 1%; width: 15%;"/>

      <InputText v-model="password" type="password" placeholder="Password" id="password" required style="margin-bottom: 1%; width: 15%;"/>
      <Button type="submit" class="buttom">Войти</button>
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
.buttom{
  width: 10%;
}
.form-login{
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 5%;
 
  
}
</style>