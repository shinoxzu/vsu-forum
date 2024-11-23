<script setup>
import { ref } from "vue";
import { Form } from "@primevue/forms";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Password from "primevue/password";
import Message from "primevue/message";

const username = ref("");
const password = ref("");
const errorMessages = ref([]);
const count = ref(0);

async function login() {
    try {
        errorMessages.value = [];

        const response = await fetch("http://localhost:3000/users/login", {
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
            console.log("Вход успешен!", data);
        } else {
            switch (response.status) {
                case 401:
                    errorMessages.value.push({
                        content: "Неверный логин или пароль",
                        id: count.value++,
                    });
                    break;
                case 500:
                    errorMessages.value.push({
                        content: "Произошла ошибка",
                        id: count.value++,
                    });
                    break;
            }
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}
</script>

<template>
    <div class="center-div">
        <h2>Вход в аккаунт</h2>
        
        <div v-if="errorMessages.length != 0" class="errors-container">
            <transition-group>
                <Message
                    icon="pi pi-times-circle"
                    v-for="msg of errorMessages"
                    :key="msg.id"
                    severity="error"
                    >{{ msg.content }}</Message
                >
            </transition-group>
        </div>
        
        <Form @submit="login" class="simple-form">
            <InputText
                v-model="username"
                id="username"
                placeholder="Имя пользователя"
                autofocus
                style="width: 100%"
                required
            />

            <Password
                placeholder="Пароль"
                v-model="password"
                :feedback="false"
                style="width: 100%"
                :input-style="{ width: '100%' }"
                toggleMask
                required
            />
            <Button type="submit">Войти</Button>
        </Form>
    </div>
</template>
