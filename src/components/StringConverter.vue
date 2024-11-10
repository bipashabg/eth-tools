<template>
  <v-card class="pa-5">
    <h3>Convert String</h3>

    <v-text-field
      label="Uneven String"
      placeholder="e.g., 0x123456"
      outlined
    ></v-text-field>

    <div class="cont">
      <v-btn color="#865DFF">Uppercase</v-btn>
      <v-btn color="#865DFF">Lowercase</v-btn>
    </div>
      <br />
      <br />
    <v-textarea
      label="Output String"
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
.cont{
  display: flex;
  justify-content: space-evenly;
}
</style>
