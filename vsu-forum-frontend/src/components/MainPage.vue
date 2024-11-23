<script setup>
import Button from "primevue/button";
import { Dialog } from "primevue";
import { ref, onMounted, watch } from "vue";
import { useRouter } from "vue-router";

const topics = ref([]);
const dialog1 = ref(false)
const dialog2 = ref(false)
const deleteRef = ref(null)
const err = ref(false)
const router = useRouter()

watch(dialog1, () => {
    if (!dialog1.value) {
        dialog2.value = true
        console.log(dialog2.value)
        deleteRef.value.play()
    }
   
})

const removePost = async (id) => {
    const token = localStorage.getItem("token");
    const resp = await fetch("http://localhost:3000/topics/" + id, {
        method: "DELETE",
        headers: {
            "Authorization":  `Bearer ${token}`
        }
    })
    if (!resp.ok) {
        console.log(resp.status)
        err.value = true
    } else {
        topics.value.filter(t => t.id != id)
    }
}

async function fetchTopics() {
    try {
        const response = await fetch("http://localhost:3000/topics");
        if (response.ok) {
            topics.value = await response.json();
        } else {
            topics.value = [];
            console.error("Ошибка при загрузке топиков");
        }
    } catch (error) {
        console.error("Ошибка сети:", error);
    }
}

const deleteClick = (e, id) => {
    dialog1.value = !dialog1.value
    removePost(id)
    e.stopPropagation()
}

onMounted(() => {
    fetchTopics();
});
</script>

<template>
    <div class="main-page">
        <div class="top-page">
            <h2>Топики</h2>
            <Button
                as="router-link"
                label="Создать топик"
                to="/create-topic"
                replace
                style="text-decoration: none"
            />
        </div>
        <div class="topics-container">
           <div
                style="text-decoration: none; color: white"
                v-for="topic in topics"
                :key="topic.id"
                @click="router.push(`/topics/${topic.id}`)"
            >
                <div class="topic">
                    <div>{{ topic.name }}
                    <Button class="delete" @click="e => deleteClick(e, topic.id)">delete</Button>
                    </div>
                </div>
            </div>
            
        </div>
    </div>
    <Dialog v-model:visible="dialog1" modal closable>
        <div>
        <video ref="deleteRef" width="1200" height="930" controls autoplay>
  <source src="../assets/delete.mp4" type="video/mp4"  >
  Your browser does not support the video tag.
</video>
</div>
    </Dialog>

    <Dialog v-model:visible="dialog2" closable position="left">
        <img style="width: 800px; height: 500px;" src="../assets/p2.png" />
    </Dialog>
    <Dialog v-model:visible="dialog2" closable position="right">
        <img style="width: 800px; height: 500px;" src="../assets/p1.png" />
    </Dialog>
    <Dialog closable v-model:visible="err" position="top"><p>Упс! Произошла ошибка</p></Dialog>
</template>

<style scoped>

.delete{
display: block;
align-self: end;
background-color: red;
color: white;
}



.topics-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.topic {
    display: flex;
    flex-direction: row;
    gap: 10px;
    background-color: #222222;
    border-radius: 7px;
    padding-left: 10px;
    padding-top: 20px;
    padding-bottom: 20px;
}

.main-page {
    padding: 20px;
}

.top-page {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
</style>
