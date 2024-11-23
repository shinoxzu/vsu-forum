<script setup>
import { ref } from "vue";
import { Form } from "@primevue/forms";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Password from "primevue/password";
import Message from "primevue/message";

const username = ref("");
const password = ref("");
const passwordConfirmation = ref("");
const errorMessages = ref([]);
const count = ref(0);

async function register() {
    try {
        errorMessages.value = [];

        if (passwordConfirmation.value != password.value) {
            return;
        }

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
            switch (response.status) {
                case 400:
                    errorMessages.value.push({
                        content: "Введены некорректные данные",
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
        <h2>Регистрация</h2>
        
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
        
        <Form @submit="register" class="simple-form">
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
                style="width: 100%"
                :input-style="{ width: '100%' }"
                toggleMask
                required
            />
            <Password
                placeholder="Подтверждение пароля"
                v-model="passwordConfirmation"
                :feedback="false"
                :invalid="password != passwordConfirmation"
                style="width: 100%"
                :input-style="{ width: '100%' }"
                toggleMask
                required
            />

            <Button type="submit">Зарегистрироваться</Button>
        </Form>
    </div>
</template>
