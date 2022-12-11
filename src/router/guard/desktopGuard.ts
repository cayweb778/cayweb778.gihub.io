import type { Router } from 'vue-router';



import {getAddr} from "../../funs";
import {useDesktopStoreWidthOut} from "../../store/modules/hello";



export function createDesktopGuard(router: Router) {
  router.beforeEach(async (to, from, next) => {

    // Jump to the 404 page after processing the login
    if(from.path === '/' &&to.fullPath==='/' &&to.fullPath==='/'){

        next('/firstPage');
        // const addrObj = await getAddr()
        // if(addrObj.code != '404'){
        //     useDesktopStoreWidthOut().setShowFirstPage(false)
        //     useDesktopStoreWidthOut().setUrl("http://"+addrObj.data+"/nc/")
        // }else{
        //     next('/firstPage');
        //     useDesktopStoreWidthOut().setShowFirstPage(true)
        // }
      return
    }else{
        next()
    }
  });
}
