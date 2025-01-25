<script setup>
import { ref, onMounted } from "vue";
import { useAuthStore } from "../stores/auth";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import Message from "primevue/message";

const authStore = useAuthStore();
const reports = ref([]);
const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const selectedReport = ref(null);
const reportedUsername = ref("");
const reportReason = ref("");
const errorMessages = ref([]);
const errorId = ref(0);

onMounted(() => {
    fetchReports();
});

async function fetchReports() {
    try {
        if (!authStore.token) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch("http://localhost:3000/reports", {
            headers: {
                Authorization: `Bearer ${authStore.token}`,
            },
        });

        if (response.ok) {
            reports.value = await response.json();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при загрузке жалоб",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
        errorMessages.value.push({
            content: "Ошибка при загрузке жалоб",
            id: errorId.value++,
        });
    }
}

async function createReport() {
    try {
        if (!authStore.token) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch("http://localhost:3000/reports", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${authStore.token}`,
            },
            body: JSON.stringify({
                reported_user_name: reportedUsername.value,
                reason: reportReason.value,
            }),
        });

        if (response.ok) {
            showCreateDialog.value = false;
            reportedUsername.value = "";
            reportReason.value = "";
            await fetchReports();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при создании жалобы",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
        errorMessages.value.push({
            content: "Ошибка при создании жалобы",
            id: errorId.value++,
        });
    }
}

function openEditDialog(report) {
    selectedReport.value = report;
    reportReason.value = report.reason;
    showEditDialog.value = true;
}

async function updateReport() {
    try {
        if (!authStore.token) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(
            `http://localhost:3000/reports/${selectedReport.value.id}`,
            {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: `Bearer ${authStore.token}`,
                },
                body: JSON.stringify({
                    reason: reportReason.value,
                }),
            },
        );

        if (response.ok) {
            showEditDialog.value = false;
            selectedReport.value = null;
            reportReason.value = "";
            await fetchReports();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при обновлении жалобы",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
        errorMessages.value.push({
            content: "Ошибка при обновлении жалобы",
            id: errorId.value++,
        });
    }
}

async function deleteReport(id) {
    try {
        if (!authStore.token) {
            errorMessages.value.push({
                content: "Необходимо войти в аккаунт",
                id: errorId.value++,
            });
            return;
        }

        const response = await fetch(`http://localhost:3000/reports/${id}`, {
            method: "DELETE",
            headers: {
                Authorization: `Bearer ${authStore.token}`,
            },
        });

        if (response.ok) {
            await fetchReports();
        } else {
            const error = await response.json();
            errorMessages.value.push({
                content: error.err || "Ошибка при удалении жалобы",
                id: errorId.value++,
            });
        }
    } catch (error) {
        console.error("Ошибка:", error);
        errorMessages.value.push({
            content: "Ошибка при удалении жалобы",
            id: errorId.value++,
        });
    }
}
</script>

<template>
    <div class="page-header">
        <h2>Жалобы на пользователей</h2>
        <Button
            label="Создать жалобу"
            icon="pi pi-plus"
            @click="showCreateDialog = true"
        />
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

    <div class="reports-list">
        <div v-for="report in reports" :key="report.id" class="report-item">
            <div class="report-content">
                <div class="report-header">
                    <span class="report-id">ID жалобы: {{ report.id }}</span>
                    <span class="user-ids">
                        Автор: {{ report.author_id }} | На пользователя:
                        {{ report.reported_user_name }}
                    </span>
                </div>
                <div class="report-reason">
                    <span>Причина:</span>
                    <p>{{ report.reason }}</p>
                </div>
            </div>
            <div class="report-actions">
                <Button
                    icon="pi pi-pencil"
                    @click="openEditDialog(report)"
                    text
                    rounded
                />
                <Button
                    icon="pi pi-trash"
                    @click="deleteReport(report.id)"
                    text
                    rounded
                    severity="danger"
                />
            </div>
        </div>
    </div>

    <Dialog v-model:visible="showCreateDialog" modal header="Создать жалобу">
        <div class="dialog-form">
            <div class="form-field">
                <label>Имя пользователя</label>
                <InputText v-model="reportedUsername" type="text" required />
            </div>
            <div class="form-field">
                <label>Причина</label>
                <Textarea v-model="reportReason" autoResize rows="3" required />
            </div>
            <Button label="Создать" @click="createReport" />
        </div>
    </Dialog>

    <Dialog
        v-model:visible="showEditDialog"
        modal
        header="Редактировать жалобу"
    >
        <div class="dialog-form">
            <div class="form-field">
                <label>Причина</label>
                <Textarea v-model="reportReason" autoResize rows="3" required />
            </div>
            <Button label="Сохранить" @click="updateReport" />
        </div>
    </Dialog>
</template>

<style scoped>
.page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.reports-list {
    display: flex;
    flex-direction: column;
    gap: 15px;
}

.report-item {
    display: flex;
    justify-content: space-between;
    align-items: start;
    padding: 15px;
    background-color: #222222;
    border-radius: 8px;
}

.report-content {
    flex-grow: 1;
}

.report-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 10px;
}

.report-id {
    font-weight: bold;
}

.user-ids {
    color: #888;
}

.report-reason {
    margin-top: 10px;
}

.report-reason span {
    font-weight: bold;
}

.report-reason p {
    margin: 5px 0;
}

.report-actions {
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
</style>
