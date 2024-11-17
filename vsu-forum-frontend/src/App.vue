<script setup>
import "primeicons/primeicons.css";
import Button from "primevue/button";
import InputText from 'primevue/inputtext';
</script>

<template>
    <header>
        <div class="title">
            <h1>Форум ВГУ</h1>
        </div>
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
        <div class="search-field">
            <InputText type="text" v-model="value" placeholder="Запрос..."/>
            <Button icon="pi pi-search" rounded aria-label="Search" />
        </div>
    </header>

    <main>
        <router-view />
    </main>
</template>

<style scoped>
header {
    display: flex;
    justify-content: start;
    align-items: center;
    padding-left: 20px;
    padding-right: 20px;
    gap: 20px;
}

.title {
    width: 20%;
}

nav {
    display: flex;
    gap: 20px;
    align-items: center;
    justify-content: start;
    width: 100%;
}

.search-field {
    margin-left: auto;
    display: flex;
    gap: 10px
}

a { 
    text-decoration: none;
}
</style>
