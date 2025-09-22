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
    label: "Products"
  },
  {
    path: "/services",
    label: "Services"
  },
  {
    path: "/about",
    label: "Pricing"
  }
]

</script>

<template>
  <header class="wrapper flex justify-between  items-center min-h-16 sm:min-h-24 bg-primary-50/10 ">
    <AppLogo class="w-36"/>

    <ul class="flex  align-center items-center gap-x-12">
      <li
          v-for="(route, index) in pageRoutes" :key=index
          class="capitalize text-[120%] hover:text-primary hover:font-medium transition-all ease-in-out">
        <NuxtLink :to="route.path"> {{ route.label }}</NuxtLink>
      </li>
    </ul>

    <div class="flex gap-x-6">
      <UButton color="transparent" size="xl"
               class="rounded-2xl  px-8 py-3 border-gray-300 border  capitalize cursor-pointer">
        Login
      </UButton>
      <UButton color="primary-300" size="xl" class="bg-primary rounded-2xl text-white px-8 py-3   cursor-pointer">
        Sign up
      </UButton>
    </div>

  </header>
</template>

<style scoped>

</style>