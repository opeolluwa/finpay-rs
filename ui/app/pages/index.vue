<script setup lang="ts">
import {UForm} from "#components";

</script>

<template>
  <main class="">
    <div
        class=" bg-linear-to-bl from-primary-50/20 to-primary-50/10 grid grid-cols-12 justify-between items-center min-h-screen align-center py-24 wrapper ">

      <div class="col-span-5">
        <h1 class="text-6xl leading-20 text-black">
          <span class="font-bold">Get paid early</span> <br/>
          save automatically <br/>
          all your pay
        </h1>
        <p class="text-xl leading-9 text-gray-500 mt-3">Support small businesses and freelancers with simple invoicing
          and powerful integrations, and cash flow
          tools </p>
        <div class="mt-3">

          <UForm >
            <UFormGroup
                label="Business Email"
                name="email"

            >
              <UInput

                  size="xl"
                  placeholder="Type your business email..."
                  class="rounded-xl py-3 px-6 bg-transparent"
              />
            </UFormGroup>

            <UButton size="xl" class="rounded-xl py-3 px-6">
              Get Started
            </UButton>
          </UForm>


        </div>
      </div>


      <div class="col-span-7">
        <img src="~/assets/img/hero.png"/>
      </div>
    </div>

  </main>
</template>

<style scoped>

</style>