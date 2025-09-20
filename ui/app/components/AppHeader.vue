<script setup lang="ts">


import AppLogo from "~/components/AppLogo.vue";

// import NuxtButton from "@nuxt/ui"

interface PageRoute {
  path: string;
  label: string;
}

const pageRoutes: PageRoute[] = [
  {
    path: "/index",
    label: "home"
  },
  {
    path: "/products",
    label: "Accountans"
  },
  {
    path: "/services",
    label: "Pricing"
  },
  {
    path: "/about",
    label: "Resources"
  }
]

</script>

<template>
  <header class="wrapper flex justify-between items-center min-h-16 sm:min-h-24 bg-gray-100">
    <AppLogo class="w-24"/>

    <ul class="hidden sm:flex gap-x-5 align-center items-center">
      <li
          v-for="(route, index) in pageRoutes" :key=index
          class="capitalize text-[120%] hover:text-app-green hover:font-medium transition-all ease-in-out">
        <NuxtLink :to="route.path"> {{ route.label }}</NuxtLink>
      </li>
    </ul>

    <div class="flex gap-x-6">
      <UButton color="transparent" size="xl" class="hidden sm:block bg-none">
        login
      </UButton>
      <UButton color="app-lime-300" size="xl" class="bg-app-lime-400 rounded-full px-6">
Sign up
      </UButton>
    </div>

  </header>
</template>

<style scoped>

</style>