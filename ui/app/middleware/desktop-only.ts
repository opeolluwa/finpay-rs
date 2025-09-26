// import {defineNuxtRouteMiddleware, navigateTo} from '#app'
// import UAParser from 'ua-parser-js'
//
// export default defineNuxtRouteMiddleware((to, from) => {
//     if (process.server) {
//         const headers = useRequestHeaders(['user-agent'])
//         const parser = new UAParser(headers['user-agent'] || '')
//         const deviceType = parser.getDevice().type
//
//         if (deviceType === 'mobile' || deviceType === 'tablet') {
//             return navigateTo('/desktop-only')
//         }
//     }
// })
