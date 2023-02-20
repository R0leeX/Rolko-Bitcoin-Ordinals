import { useState } from 'react';
import styles from "../styles/DropZone.module.css";


function ValidateAddress() {
  fetch('/api/validate-address', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ address: bitcoinAddress })
  })
  .then(response => {
    if (response.ok) {
      GenerateInvoices();
    } else {
      throw new Error('Failed to submit address');
    }
  })
  .catch(error => {
    alert(`Error submitting address: ${error.message}`);
  });
}

function GenerateInvoices() {
  fetch('/api/generate-invoices', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ address: bitcoinAddress })
  })
  .then(response => {
    if (response.ok) {
      //TODO: Show invoice on website.
    } else {
      throw new Error('Failed to submit address');
    }
  })
  .catch(error => {
    alert(`Error submitting address: ${error.message}`);
  });
}

function BitcoinAddressForm() {
  const [bitcoinAddress, setBitcoinAddress] = useState('');

  const handleSubmit = (event) => {
    event.preventDefault();
    ValidateAddress();
  }

  return (
    <form onSubmit={handleSubmit}>
    <p >Enter the BTC address you want the inscription sent: </p>
      <input
        id="bitcoin-address"
        type="text"
        value={bitcoinAddress}
        onChange={(event) => setBitcoinAddress(event.target.value)}
        placeholder="" style={{width: "510px", color: "black"}}
        required
      />
      <button className={styles.uploadBtn} onClick={handleSubmit}>
          Submit</button>
    </form>
  );
}

export default BitcoinAddressForm;