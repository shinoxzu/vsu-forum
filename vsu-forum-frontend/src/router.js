import { createRouter, createWebHistory } from "vue-router";
import UserRegister from "./components/UserRegister.vue";
import UserLogin from "./components/UserLogin.vue";
import UserProfile from "./components/UserProfile.vue";
import MainPage from "./components/MainPage.vue";
import TopicPage from "./components/TopicPage.vue";
import TopicCreationPage from "./components/TopicCreationPage.vue";

const routes = [
    { path: "/", component: MainPage, name: "Home" },
    { path: "/create-topic", component: TopicCreationPage, name: "CreateTopic" },
    { path: "/register", component: UserRegister, name: "Register" },
    { path: "/login", component: UserLogin, name: "Login" },
    { path: "/profile", component: UserProfile, name: "Profile" },
    { path: "/topics/:id", component: TopicPage, name: "Topic", props: true},
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
