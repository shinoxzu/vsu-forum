<script setup>
import Button from 'primevue/button';


</script>

<template>
  
  <header>
    <h1>Форум ВГУ</h1>
    <nav>
      <Button as="router-link" target="_blank" rel="noopener" label="Главная" to="/" />
      <Button as="router-link" target="_blank" rel="noopener" label="Регистрация" to="/register" />
      <Button as="router-link" target="_blank" rel="noopener" label="Вход" to="/login" />
      <Button as="router-link" target="_blank" rel="noopener" label="Профиль" to="/profile" />
    </nav>
  </header>

  <main>
    <router-view />
  </main>
</template>

<script>

export default {
  name: 'App',
};
</script>

<style scoped>
header {
  background-color: #42b983;
  color: white;
  padding: 20px;
}
nav {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
  width: 100%;
}
</style>
