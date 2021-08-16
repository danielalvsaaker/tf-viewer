const Upload = {
    props: ["user"],
    template: `
            <div class="new">
                <form id="newpost" method="POST" enctype=mulitpart/form-data v-on:submit="publishPost">
                    <h4>Upload</h4>
                    <label for="fileupload">Upload </label>
                    <p v-if="error">{{ error }}</p>
                    <input type="file" ref="uploadedFiles" name="fileupload" multiple>
                    <input type="submit" value="Upload">
                </form>
            </div>
    `,
    data: function(){
        return {
            files: [],
            error: null
        }
    },
    methods:{
        publishPost: async function() {
            event.preventDefault();
            this.error = null;
            this.files = this.$refs.uploadedFiles.files;
            let formData = new FormData();
            formData.append("activity",this.files[0], this.files[0].name);
            let response = await fetch("/user/asd/activity", {
                body: formData,
                method: "POST",
            });
            let result = await response;
            console.log(response);
            this.$refs.uploadedFiles.value = null;
        }
    }
}
