<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "./stores/auth";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Dialog from "primevue/dialog";

const router = useRouter();
const authStore = useAuthStore();
const searchQuery = ref("");
const showSearchResults = ref(false);
const searchResults = ref([]);
const isSearching = ref(false);
const searchError = ref(null);
const showMobileMenu = ref(false);

async function performSearch() {
    if (!searchQuery.value.trim()) return;

    try {
        isSearching.value = true;
        searchError.value = null;

        const response = await fetch(
            `http://localhost:3000/search?query=${encodeURIComponent(
                searchQuery.value,
            )}`,
        );

        if (response.ok) {
            searchResults.value = await response.json();
            showSearchResults.value = true;
        } else {
            const error = await response.json();
            searchError.value = error.err || "Ошибка при поиске";
        }
    } catch (error) {
        console.error("Search error:", error);
        searchError.value = "Произошла ошибка при поиске";
    } finally {
        isSearching.value = false;
    }
}

function navigateToTopic(topicId) {
    router.push(`/topics/${topicId}`);
    showSearchResults.value = false;
    searchQuery.value = "";
}

function toggleMenu() {
    showMobileMenu.value = !showMobileMenu.value;
}
</script>

<template>
    <header>
        <div class="title">
            <h1>Форум ВГУ</h1>
            <Button icon="pi pi-bars" @click="toggleMenu" class="menu-toggle" />
        </div>
        <nav :class="{ 'nav-mobile': showMobileMenu }">
            <Button as="router-link" label="Главная" to="/" />
            <template v-if="!authStore.isAuthorized">
                <Button as="router-link" label="Регистрация" to="/register" />
                <Button as="router-link" label="Вход" to="/login" />
            </template>
            <template v-else>
                <Button as="router-link" label="Профиль" to="/profile" />
                <Button as="router-link" label="Закладки" to="/bookmarks" />
                <Button as="router-link" label="Жалобы" to="/reports" />
                <Button
                    as="router-link"
                    label="Реакции"
                    to="/available-reactions"
                />
            </template>
            <Button as="router-link" label="Статистика" to="/stats" />
        </nav>
        <div class="search-field">
            <InputText
                v-model="searchQuery"
                type="text"
                placeholder="Поиск..."
                @keyup.enter="performSearch"
            />
            <Button
                icon="pi pi-search"
                rounded
                aria-label="Search"
                :loading="isSearching"
                @click="performSearch"
            />
        </div>
    </header>

    <Dialog
        v-model:visible="showSearchResults"
        modal
        header="Результаты поиска"
        :style="{ width: '90vw', maxWidth: '600px' }"
    >
        <div v-if="searchError" class="search-error">
            {{ searchError }}
        </div>
        <div v-else-if="searchResults.length === 0" class="no-results">
            Ничего не найдено
        </div>
        <div v-else class="search-results">
            <div
                v-for="topic in searchResults"
                :key="topic.id"
                class="search-result-item"
                @click="navigateToTopic(topic.id)"
            >
                <h3>{{ topic.name }}</h3>
                <div class="topic-details">
                    <span>Категория: {{ topic.category.name }}</span>
                    <span>Автор: {{ topic.creator.login }}</span>
                    <span>Постов: {{ topic.posts_count }}</span>
                </div>
            </div>
        </div>
    </Dialog>

    <main>
        <router-view />
    </main>
</template>

<style scoped>
header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    background-color: #1a1818;
}

.title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
}

.menu-toggle {
    display: none;
}

nav {
    display: flex;
    gap: 10px;
    align-items: center;
}

.search-field {
    display: flex;
    gap: 10px;
}

@media (max-width: 768px) {
    .menu-toggle {
        display: block;
    }

    nav {
        display: none;
        width: 100%;
        flex-direction: column;
        align-items: stretch;
        padding: 10px 0;
    }

    nav.nav-mobile {
        display: flex;
    }

    .search-field {
        width: 100%;
        margin-top: 10px;
    }

    .search-field :deep(input) {
        width: 100%;
    }
}

.search-results {
    display: flex;
    flex-direction: column;
    gap: 15px;
    max-height: 60vh;
    overflow-y: auto;
}

.search-result-item {
    padding: 15px;
    background-color: #222222;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.search-result-item:hover {
    background-color: #2a2a2a;
}

.search-result-item h3 {
    margin: 0 0 10px 0;
}

.topic-details {
    display: flex;
    flex-wrap: wrap;
    gap: 15px;
    color: #888;
    font-size: 0.9em;
}

.search-error {
    color: #ef4444;
    text-align: center;
    padding: 20px;
}

.no-results {
    text-align: center;
    padding: 20px;
    color: #888;
}

a {
    text-decoration: none;
}

main {
    padding: 15px;
}
</style>
