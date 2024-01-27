<script setup>

import { Head, Link } from '@inertiajs/vue3';

defineProps({
    foo: {
        type: String,
        required: true,
    },
})

</script>
<template>
    <Head title="Hello, world!" />
    <main class="fixed inset-0 grid place-items-center">
         <h1 class="text-2xl">Hello, {{ foo }}!</h1>
         <Link href="/" class="text-blue-600 visited:text-purple-600">Refresh</Link>
    </main>
</template>