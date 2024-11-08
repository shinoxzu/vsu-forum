<template>
  <header>
    <h1>Форум ВГУ</h1>
    <nav>
      <router-link to="/">Главная</router-link>
      <router-link to="/register">Регистрация</router-link>
      <router-link to="/login">Вход</router-link>
      <router-link to="/profile">Профиль</router-link>
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
}
</style>
