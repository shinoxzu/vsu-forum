<template>
  <div class="main-page">
    <h2>Топики форума</h2>
    <ul v-if="topics.length">
      <li v-for="topic in topics" :key="topic.id">
        {{ topic.title }}
      </li>
    </ul>
    <p v-else>Топики пока не добавлены.</p>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';

export default {
  name: 'MainPage',
  setup() {
    const topics = ref([]);

    async function fetchTopics() {
      try {
        const response = await fetch('http://localhost:3000/topics');
        if (response.ok) {
          topics.value = await response.json();
        } else {
          console.error("Ошибка при загрузке данных");
        }
      } catch (error) {
        console.error("Ошибка сети:", error);
      }
    }

    onMounted(fetchTopics);

    return {
      topics
    };
  }
};
</script>

<style scoped>
.main-page {
  padding: 20px;
}
</style>