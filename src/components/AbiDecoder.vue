<template>
  <v-card class="pa-5">
    <h3>ABI Decoder</h3>

    <v-text-field
      label="ABI"
      placeholder="e.g., [{ 'type': 'uint256', 'name': 'value' }]"
      v-model="abiTypes"
      outlined
    ></v-text-field>

    <v-textarea
      label="Input Data"
      placeholder="e.g., 0x123456"
      v-model="parameters"
      outlined
      rows="3"
    ></v-textarea>

    <v-btn color="#865DFF" @click="encodeAbi">Decode</v-btn>
      <br />
      <br />
    <v-textarea
      label="Decoded Output"
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
