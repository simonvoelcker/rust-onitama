import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Card extends Component {

  getClasses () {
    return this.props.mini ? 'card mini-card' : 'card'
  }

  getImageSrc () {
    return 'http://localhost:3030/static/card-' + this.props.name + '.jpg'
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <img className={this.getClasses()} src={this.getImageSrc()} alt='card' />
        )}
      </AppConsumer>
    )
  }
}

Card.contextType = AppContext
