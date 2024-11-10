<template>
  <v-card class="pa-5">
    <h3>Block to TimeStamp Converter</h3>

    <v-text-field
      label="Block Number"
      placeholder="e.g., 123456"
      outlined
    ></v-text-field>

    <v-btn color="#865DFF" @click="encodeAbi">Show TimeStamp</v-btn>
      <br />
      <br />
    <v-textarea
      label="Time Stamp"
      outlined
      rows="5"
      readonly
    ></v-textarea>
  </v-card>
</template>

<script>
</script>

<style scoped>
h3 {
  font-weight: bold;
  margin-bottom: 20px;
  font-family: 'Dongle', sans-serif;
}
</style>
