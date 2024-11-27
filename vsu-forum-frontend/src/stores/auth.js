import { ref, computed } from "vue";
import { defineStore } from "pinia";

export const useAuthStore = defineStore("auth", () => {
    const token = ref(localStorage.getItem("token"));

    const isAuthorized = computed(() => token.value !== null);

    function setToken(newToken) {
        token.value = newToken;
        if (newToken) {
            localStorage.setItem("token", newToken);
        } else {
            localStorage.removeItem("token");
        }
    }

    return {
        token,
        isAuthorized,
        setToken,
    };
});
