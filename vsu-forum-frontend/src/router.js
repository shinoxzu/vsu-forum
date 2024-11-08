import { createRouter, createWebHistory } from 'vue-router';
import UserRegister from './components/UserRegister.vue';
import UserLogin from './components/UserLogin.vue';
import UserProfile from './components/UserProfile.vue';
import MainPage from './components/MainPage.vue';

const routes = [
  { path: '/', component: MainPage, name: 'Home' },
  { path: '/register', component: UserRegister, name: 'Register' },
  { path: '/login', component: UserLogin, name: 'Login' },
  { path: '/profile', component: UserProfile, name: 'Profile' },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;