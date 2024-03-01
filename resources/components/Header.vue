<template>
  <nav id="header" class="bg-white fixed w-full z-10 top-0 shadow">
    <div class="w-full container mx-auto flex flex-wrap items-center mt-0 pt-3 pb-3 md:pb-0">
      <div class="w-full pl-2 md:pl-0">
        <NuxtLink to="/" class="!text-gray-900 text-base xl:text-xl no-underline hover:no-underline font-bold">
          <font-awesome-icon icon="fa-solid fa-cubes" class="text-sky-600 pr-3"/>
          HarbUI - Docker Registry UI
        </NuxtLink>
      </div>
      <div class="w-full flex-grow lg:flex lg:items-center lg:w-auto hidden lg:block mt-2 lg:mt-0 bg-white z-20"
           id="nav-content">
        <ul class="list-reset lg:flex flex-1 items-center px-4 md:px-0 text-gray-500">
          <li class="mr-6 my-2 md:my-0">
            <NuxtLink to="/"
                      class="block py-1 md:py-3 pl-1 align-middle no-underline border-b-2 hover:border-sky-500 hover:text-sky-600">
              <font-awesome-icon icon="fa-solid fa-hard-drive" class="mr-3"/>
              <span class="pb-1 md:pb-0 text-sm">Images</span>
            </NuxtLink>
          </li>
          <li class="mr-6 my-2 md:my-0">
            <NuxtLink to="/pulling"
                      class="block py-1 md:py-3 pl-1 align-middle no-underline border-b-2 hover:border-sky-500 hover:text-sky-600">
              <font-awesome-icon icon="fa-solid fa-cloud-arrow-up" class="mr-3"/>
              <span class="pb-1 md:pb-0 text-sm">Pulling</span>
            </NuxtLink>
          </li>
          <li class="mr-6 my-2 md:my-0">
            <NuxtLink to="/pushing"
                      class="block py-1 md:py-3 pl-1 align-middle no-underline border-b-2 hover:border-sky-500 hover:text-sky-600">
              <font-awesome-icon icon="fa-solid fa-cloud-arrow-down" class="mr-3"/>
              <span class="pb-1 md:pb-0 text-sm">Pushing</span>
            </NuxtLink>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>