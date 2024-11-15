<script setup>
import Button from "primevue/button";
</script>

<template>
    <header>
        <h1>Форум ВГУ</h1>
        <nav>
            <Button as="router-link" label="Главная" to="/" replace />
            <Button
                as="router-link"
                label="Регистрация"
                to="/register"
                replace
            />
            <Button as="router-link" label="Вход" to="/login" replace />
            <Button as="router-link" label="Профиль" to="/profile" replace />
        </nav>
    </header>

    <main>
        <router-view />
    </main>
</template>

<script>
export default {
    name: "App",
};
</script>

<style scoped>
header {
    background-color: #6208b1a0;
    color: white;
    padding: 20px;
    width: 100%;
}
nav {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
    width: 100%;
}
</style>
