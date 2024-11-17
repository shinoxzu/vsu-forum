<script setup>
import { ref } from "vue";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
const username = ref("");
const password = ref("");

async function register() {
    try {
        const response = await fetch("http://localhost:3000/users/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                login: username.value,
                password: password.value,
            }),
        });
        if (response.ok) {
            const data = await response.json();
            localStorage.setItem("token", data.token);
            console.log("Регистрация успешна!", data);
        } else {
            console.error("Ошибка при регистрации");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}
</script>

<template>
    <div class="register">
        <form @submit.prevent="register" class="register_form">
            <h2>Регистрация</h2>
            <InputText
                v-model="username"
                id="username"
                placeholder="Username"
                required
                style="margin-bottom: 1%; width: 15%"
            />

            <InputText
                v-model="password"
                type="password"
                placeholder="Password"
                id="password"
                required
                style="margin-bottom: 1%; width: 15%"
            />
            <Button type="submit">Зарегистрироваться</Button>
        </form>
    </div>
</template>


    <style scoped>
    .buttom {
        width: 10%;
    }
    .register_form {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 5%;
    }
    </style>