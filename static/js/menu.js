const Menu = {
    props: ["user"],
    template: `
            <div class="menugroup_left menugroup">
                <router-link to="/"><h3>TF-Viewer</h3></router-link>
            </div>
            <div class="menugroup_middle menugroup">
                <router-link to="/activitytable"><i class="fa fa-tree fa-2x"></i></router-link>
                <router-link to="/equipment"><i class="fa fa-bicycle fa-2x"></i></router-link>
                <router-link to="/upload"><i class="fa fa-upload fa-2x"></i></router-link>
            </div>
            <div class="menugroup_right menugroup">
                <p v-if="user" id="username">{{ user }}</p>
                <router-link v-if="user" to="/"><i class="fa fa-cog fa-2x"></i></router-link>
                <a v-if="user" href="#" v-on:click="logout"><i class="fa fa-sign-out fa-2x"></i></a>
                <router-link v-if="!user" to="/login"><i class="fa fa-sign-in fa-2x"></i></router-link>
                <router-link v-if="!user" to="/register"><i class="fa fa-user-plus fa-2x"></i></router-link>
            </div>
            `,

}
