<template>
    <div>
      <h2>{{ title }}</h2>

      <div v-if="title === 'ABI Encoder/Decoder'" class="abi-tool">
        <v-card class="mb-5" app color = "#191825">
          <v-card-title>Encoder</v-card-title>
          <v-card-text>
            <v-textarea
              label="Input for Encoding"
              v-model="encoderInput"
              placeholder="e.g., ['address', 'uint256']"
              rows="4"
            ></v-textarea>
            <v-btn @click="encodeInput" color="#865DFF" class="mt-3">Encode</v-btn>
            <v-textarea
              label="Encoded Output"
              v-model="encoderOutput"
              readonly
              rows="4"
              class="mt-3"
            ></v-textarea>
          </v-card-text>
        </v-card>

        <v-card>
          <v-card-title>Decoder</v-card-title>
          <v-card-text>
            <v-textarea
              label="Input for Decoding"
              v-model="decoderInput"
              placeholder="e.g., '0x...'"
              rows="4"
            ></v-textarea>
            <v-btn @click="decodeInput" color="primary" class="mt-3">Decode</v-btn>
            <v-textarea
              label="Decoded Output"
              v-model="decoderOutput"
              readonly
              rows="4"
              class="mt-3"
            ></v-textarea>
          </v-card-text>
        </v-card>
      </div>

      <div v-else>

        <v-select
          :items="conversions"
          label="Select Conversion Type"
          v-model="selectedConversion"
          class="mb-3"
        ></v-select>
        <v-textarea label="Input" v-model="inputValue" rows="4"></v-textarea>
        <v-btn @click="convert" color="primary" class="mt-3">Convert</v-btn>
        <v-textarea
          label="Output"
          v-model="outputValue"
          readonly
          rows="4"
          class="mt-3"
        ></v-textarea>
      </div>
    </div>
  </template>

  <script>
  export default {
    props: {
      title: String,
      conversions: Array
    },
    data() {
      return {
        encoderInput: '',
        encoderOutput: '',
        decoderInput: '',
        decoderOutput: '',
        selectedConversion: null,
        inputValue: '',
        outputValue: ''
      };
    },
    methods: {
      encodeInput() {

        this.encoderOutput = `Encoded: ${this.encoderInput}`;
      },
      decodeInput() {

        this.decoderOutput = `Decoded: ${this.decoderInput}`;
      },
      convert() {

        this.outputValue = `Converted: ${this.inputValue}`;
      }
    }
  };
  </script>

  <style scoped>
  .abi-tool {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
    .v-btn{
      background-color: #865DFF;
    }

  </style>
