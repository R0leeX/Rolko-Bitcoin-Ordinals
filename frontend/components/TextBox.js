import { useState } from 'react';
import styles from "../styles/DropZone.module.css";

function BitcoinAddressForm() {
  const [bitcoinAddress, setBitcoinAddress] = useState('');

  const handleSubmit = (event) => {
    event.preventDefault();
    alert(`Submitted address: ${bitcoinAddress}`);
    // TODO: Send it to backend
  }

  return (
    <form onSubmit={handleSubmit}>
    <p >Enter the BTC address you want the inscription sent: </p>
      <input
        id="bitcoin-address"
        type="text"
        value={bitcoinAddress}
        onChange={(event) => setBitcoinAddress(event.target.value)}
        placeholder="" style={{width: "510px"}}
        required
      />
      <button className={styles.uploadBtn} onClick={handleSubmit}>
          Submit</button>
    </form>
  );
}

export default BitcoinAddressForm;