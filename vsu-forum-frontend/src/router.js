import { createRouter, createWebHistory } from "vue-router";
import UserRegister from "./components/UserRegister.vue";
import UserLogin from "./components/UserLogin.vue";
import UserProfile from "./components/UserProfile.vue";
import MainPage from "./components/MainPage.vue";
import TopicPage from "./components/TopicPage.vue";
import TopicCreationPage from "./components/TopicCreationPage.vue";
import AvailableReactions from "./components/AvailableReactions.vue";
import BookmarkedTopics from "./components/BookmarkedTopics.vue";
import ReportsPage from "./components/ReportsPage.vue";

const routes = [
    { path: "/", component: MainPage, name: "Home" },
    { path: "/create-topic", component: TopicCreationPage, name: "CreateTopic" },
    { path: "/register", component: UserRegister, name: "Register" },
    { path: "/login", component: UserLogin, name: "Login" },
    { path: "/profile", component: UserProfile, name: "Profile" },
    { path: "/available-reactions", component: AvailableReactions, name: "AvailableReaction" },
    { path: "/topics/:id", component: TopicPage, name: "Topic", props: true},
    { path: "/bookmarks", component: BookmarkedTopics, name: "Bookmarks" },
    { path: "/reports", component: ReportsPage, name: "Reports" }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
