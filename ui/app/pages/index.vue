<script setup lang="ts"></script>

<template>
  <main class="">
    <div
        class="bg-primary-50/10 flex grid-cols-12 justify-between items-center min-h-screen align-center py-24 wrapper pb-80"
    >
      <div class="col-span-7 flex flex-col gap-y-5 sm:gap-y-10">
        <h1 class="text-3xl sm:text-6xl leading-10 sm:leading-22 font-medium">
          <span class="font-extrabold">Get paid early</span> <br/>
          save automatically <br/>
          all your pay
        </h1>
        <p class="sm:text-xl leading-7 sm:leading-9 text-gray-500">
          Support small businesses and freelancers with simple invoicing and
          powerful integrations, and cash flow tools
        </p>
        <div class="mt-3">
          <form
              class="rounded-xl grid grid-cols-12 bg-white border-gray-300 border-2 hover:border-primary"
          >
            <input
                size="xl"
                placeholder="Type your business email..."
                class="block col-span-8 py-1 sm:py-3 px-6 bg-transparent border-0 focus:ring-0 focus:border-0 shadow-none focus:outline-0 hover:outline-none"
            />

            <UButton
                size="xl"
                class="rounded-xl block -3 px-6 col-span-4 my-[1px]"
            >
              Get Started
              <UIcon name="heroicons:arrow-up-right" class="size-5"/>
            </UButton>
          </form>

          <div
              class="grid grid-cols-12 justify-between items-center gap-x-8 align-center mt-10"
          >
            <img src="~/assets/img/klarna.png" class="col-span-4"/>
            <img src="~/assets/img/coinbase.png" class="col-span-4"/>
            <img src="~/assets/img/instacart.png" class="col-span-4"/>
          </div>
        </div>
      </div>

      <div class="col-span-5">
        <img src="~/assets/img/hero.png"/>
      </div>
    </div>

    <div
        class="wrapper bg-white rounded-3xl grid grid-rows-2 grid-cols-12 items-center gap-10 mx-20 py-12 shadow-2xl relative -top-60 mt-20"
    >
      <div class="row-span-1 col-span-6">
        <p class="text-primary capitalize">FUTURE PAYMENT</p>
        <h2 class="leading-20 text-5xl font-medium">
          Experience that grows with your scale
        </h2>
      </div>
      <div class="row-span-1 col-span-6 text-xl leading-10">
        Design a financial operating system that works for your business and
        streamlines cash flow management
      </div>
      <div class="feature">
        <img src="~/assets/img/transfer.png"/>
        <h3 class="medium capitalize mt-b">Free transfers</h3>
        <p>
          Create a financial experience and automate repeated purchases by
          scheduling purchases and recoring payments
        </p>
      </div>

      <div class="feature">
        <img src="~/assets/img/accounts.png"/>

        <h3 class="medium capitalize mt-b">Multiple accounts</h3>
        <p>
          Run your operation with cash from your accounts and generate yield
          from funds stored in your account
        </p>
      </div>

      <div class="feature">
        <img src="~/assets/img/security.png"/>

        <h3 class="medium capitalize mt-b">Unmatched security</h3>
        <p>
          CSecurely manage your finances with organization-wide MFA, card
          locking and account level controls
        </p>
      </div>
    </div>

    <div class="wrapper bg-white">
      <p class="text-primary uppercase text-center">why us</p>
      <h2 class="leading-20 text-5xl font-medium text-center">
        Why they prefer Finpay
      </h2>
      <div
          class="grid grid-rows-2 grid-cols-12 items-center gap-x-20 py-12 relative mt-8"
      >
        <div
            class="col-span-6 row-span-1 bg-primary-50/10 wrapper py-12 rounded-3xl"
        >
          <h4 class="text-9xl font-bold text-primary">3k+</h4>
          <p class="mt-12 font-medium text-4xl">Business running on Finpay</p>
        </div>

        <div
            class="col-span-6 row-span-1 bg-primary-50/10 wrapper py-12 rounded-3xl"
        >
          <p class="mt-12 font-medium text-4xl">
            Instant withdraw of your funds at any time
          </p>
          <img src="~/assets/img/withdraw.png" class="mt-10"/>
        </div>

        <div
            class="row-span-1 col-span-12 flex items-center bg-primary-50/10 wrapper py-12 rounded-3xl"
        >
          <div class="">
            <h4 class="text-3xl font-bold">No assets volatility</h4>
            <p class="mt-8 text-xl leading-8">
              Generate returns on your cash reserves without making any
              investments
            </p>
          </div>
          <div>
            <img
                src="~/assets/img/investmenent.png"
                class="rounded-xl shadow-xl"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="bg-primary-700 text-white py-12 wrapper">
      <div class="w-1/2">
        <p class="text-primary capitalize">STEP</p>
        <h2 class="leading-20 text-5xl font-medium">
          Experience that grows with your scale
        </h2>
      </div>

      <div class="grid grid-cols-12 gap-x-4 justify-between w-full">
        <div class="steps">
          <h3 class="text-7xl">1</h3>
          <div>
            <h4>Open account</h4>
            <p>
              Transfer money from one account into another and start earning up
            </p>
          </div>
        </div>

        <div class="steps">
          <h3 class="text-7xl">2</h3>
          <div>
            <h4>Move money</h4>
            <p>
              Transfer money from one account into another and start earning up
            </p>
          </div>
        </div>

        <div class="steps">
          <h3 class="text-7xl">3</h3>
          <div>
            <h4>Watch you balance grow</h4>
            <p>
              Accessed instantly and remained insulated from market volatility
            </p>
          </div>
        </div>
      </div>
    </div>

    <div class="bg-white mt-20">
      <p class="text-primary uppercase text-center">OUT MISSION</p>
      <h2 class="leading-20 text-5xl font-medium text-center">
        We&apos; helped </br> imprved business
      </h2>
    </div>
  </main>
</template>

<style scoped></style>
