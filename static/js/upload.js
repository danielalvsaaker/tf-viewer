const Upload = {
    props: ["user", "site"],
    template: `
            <div class="new">
                <form id="newpost" method="POST" enctype=mulitpart/form-data v-on:change="uploadFiles">
                    <h4>Upload</h4>
                    <p v-if="error">{{ error }}</p>
                    <input type="file" ref="uploadedFiles" name="fileupload" multiple>
                    <p v-if="files.length">{{ uploadCount }} / {{ files.length }}</p>
                </form>
                <div class="file" v-for="file in files" >
                    <p>{{ file.name }}</p>
                    <i v-if="uploadStatus[file.name] == null" class="fa fa-hourglass-half"></i>
                    <i v-else-if="uploadStatus[file.name]" class="fa fa-check"></i>
                    <i v-else-if="!uploadStatus[file.name]" class="fa fa-exclamation-circle"></i>
                </div>
            </div>
    `,
    data: function(){
        return {
            files: [],
            uploadStatus: {},
            uploadCount: 0,
            error: null,
            startTime: new Date()
        }
    },
    methods:{
        uploadFiles: async function() {
            event.preventDefault();
            this.startTime = new Date()
            this.files = [];
            this.uploadStatus = {};
            this.error = null;
            this.files = this.$refs.uploadedFiles.files;
            let promises = [];
            for (let i = 0; i < this.files.length; i++) {
                const file = this.files[i];
                promises.push(upload(this.uploadStatus, file));
                if ((i+1) % 100 == 0) {
                    await Promise.all(promises);
                    promises = [];
                    this.uploadCount = i+1;
                }
            }
            await Promise.all(promises);
            this.uploadCount = this.files.length;
            let endTime = new Date();
            console.log(endTime.getTime() - this.startTime.getTime());
            //this.$refs.uploadedFiles.value = null;
        },
    }
}

async function upload(statusObject, file) {
    let response = await fetch("/user/asd/activity", {
        body: file,
        method: "POST",
    });
    let result = await response;
    if (result.status == 201) {
        statusObject[file.name] = true;
    } else {
        statusObject[file.name] = false;
    }
    console.log(response.headers.get("Location"));
    return Promise.resolve();
}
