<script setup>
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import { ref, onMounted } from "vue";

const reactions = ref([]);
const showDialog = ref(false);
const isEditing = ref(false);
const selectedId = ref(null);
const reactionToEdit = ref("");
const errorMessages = ref([]);
const errorId = ref(0);

async function fetchReactions() {
    try {
        const response = await fetch(
            "http://localhost:3000/available-reactions",
        );
        if (response.ok) {
            reactions.value = await response.json();
        } else {
            errorMessages.value.push({
                content: "Ошибка при загрузке реакций",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
    }
}

async function saveReaction() {
    try {
        const url = isEditing.value
            ? `http://localhost:3000/available-reactions/${selectedId.value}`
            : "http://localhost:3000/available-reactions";

        const method = isEditing.value ? "PATCH" : "POST";

        const response = await fetch(url, {
            method,
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${localStorage.getItem("token")}`,
            },
            body: JSON.stringify({ reaction: reactionToEdit.value }),
        });

        if (response.ok) {
            showDialog.value = false;
            reactionToEdit.value = "";
            await fetchReactions();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Произошла ошибка",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
    }
}

async function deleteReaction(id) {
    try {
        const response = await fetch(
            `http://localhost:3000/available-reactions/${id}`,
            {
                method: "DELETE",
                headers: {
                    Authorization: `Bearer ${localStorage.getItem("token")}`,
                },
            },
        );

        if (response.ok) {
            await fetchReactions();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при удалении",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
    }
}

function openCreateDialog() {
    isEditing.value = false;
    reactionToEdit.value = "";
    showDialog.value = true;
}

function openEditDialog(reaction) {
    isEditing.value = true;
    selectedId.value = reaction.id;
    reaction.value = reaction.reaction;
    showDialog.value = true;
}

onMounted(async () => {
    await fetchReactions();
});
</script>

<template>
    <div class="page-header">
        <h2>Реакции</h2>
        <Button label="Добавить" icon="pi pi-plus" @click="openCreateDialog" />
    </div>

    <div v-if="errorMessages.length > 0" class="errors-container">
        <Message
            v-for="msg of errorMessages"
            :key="msg.id"
            severity="error"
            :life="3000"
        >
            {{ msg.content }}
        </Message>
    </div>

    <div class="reactions-list">
        <div
            v-for="reaction in reactions"
            :key="reaction.id"
            class="reaction-item"
        >
            <div class="reaction-content">
                <span class="reaction-reaction">{{ reaction.reaction }}</span>
            </div>
            <div class="reaction-actions">
                <Button
                    icon="pi pi-pencil"
                    class="p-button-rounded p-button-text"
                    @click="openEditDialog(reaction)"
                />
                <Button
                    icon="pi pi-trash"
                    class="p-button-rounded p-button-text p-button-danger"
                    @click="deleteReaction(reaction.id)"
                />
            </div>
        </div>
    </div>

    <Dialog
        v-model:visible="showDialog"
        :modal="true"
        :header="isEditing ? 'Изменить реакцию' : 'Добавить реакцию'"
    >
        <div class="dialog-form">
            <div class="form-field">
                <label>Реакция</label>
                <InputText v-model="reactionToEdit" required />
            </div>
            <Button
                :label="isEditing ? 'Сохранить' : 'Создать'"
                @click="saveReaction"
            />
        </div>
    </Dialog>
</template>

<style scoped>
.page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}

.reactions-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.reaction-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px;
    background-color: #222222;
    border-radius: 8px;
}

.reaction-content {
    display: flex;
    align-items: center;
    gap: 10px;
}

.reaction-reaction {
    font-size: 1.5em;
}

.reaction-actions {
    display: flex;
    gap: 5px;
}

.dialog-form {
    display: flex;
    flex-direction: column;
    gap: 15px;
    padding: 20px;
}

.form-field {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.errors-container {
    margin-bottom: 15px;
}
</style>
