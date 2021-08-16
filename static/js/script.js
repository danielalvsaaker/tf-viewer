const routes = [
    { path: '/', component: MainSite, props: {site: "main"}},
    { path: '/login', component: Login, props: {site: "login"}},
    { path: '/register', component: Register, props: {site: "register"}},
    { path: '/activitytable', component: ActivityTable, props: {site: "activitytable"}},
    { path: '/equipment', component: Equipment, props: {site: "equipment"}},
    { path: '/upload', component: Upload, props: {site: "upload"}},
];
const router = VueRouter.createRouter({
    history: VueRouter.createWebHashHistory(),
    routes: routes
});

//the get* functions defined here are used in serveral different components and are therefore not just a mehthod within the component.
async function getUsername() {
    let response = await fetch('/username', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    });
    let result = await response.json()
    let username = result["username"];
    return username;
}



function pathString(path) {
    let pathList = ["global", "user", "login", "register", "settings"]
    for (let i = 0; i < pathList.length; i++) {
        if (path.includes(pathList[i])) {
            return pathList[i];
        }
    }
    return "main"

}

let app = Vue.createApp({
    data: function(){
        return {
            user: null,
        }
    }
    //needed for fetching username when page is refreshed
    //created: async function() {
        //this.user = await getUsername();
    //}
});
app.use(router);
app.component('mainsite', MainSite);
app.component('menubar', Menu);
app.component('login', Login);
app.component('register', Register);
app.component('activitytable', ActivityTable);
app.component('equipment', Equipment);
app.component('upload', Upload);
app.mount("#app");
