import { useState } from 'react';
import styles from "../styles/DropZone.module.css";

function BitcoinAddressForm() {
  const [bitcoinAddress, setBitcoinAddress] = useState('');

  const handleSubmit = (event) => {
    event.preventDefault();
    fetch('/api/send-address', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ address: bitcoinAddress })
    })
    .then(response => {
      if (response.ok) {
        alert('Address submitted successfully!');
      } else {
        throw new Error('Failed to submit address');
      }
    })
    .catch(error => {
      alert(`Error submitting address: ${error.message}`);
    });
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